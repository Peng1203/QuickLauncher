import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import process from 'node:process';
import { confirm, input, select } from '@inquirer/prompts';
import { config } from 'dotenv';

// 加载 .env 文件
config({ path: join(resolve(import.meta.dirname, '..'), '.env.local') });

const ROOT_DIR = resolve(import.meta.dirname, '..');
const PACKAGE_JSON_PATH = join(ROOT_DIR, 'package.json');
const LATEST_JSON_PATH = join(ROOT_DIR, 'latest.json');
const CARGO_TOML_PATH = join(ROOT_DIR, 'src-tauri', 'Cargo.toml');
const TAURI_CONF_PATH = join(ROOT_DIR, 'src-tauri', 'tauri.conf.json');

const TAURI_SIGNING_PRIVATE_KEY = process.env.TAURI_SIGNING_PRIVATE_KEY;
const TAURI_SIGNING_PRIVATE_KEY_PASSWORD = process.env.TAURI_SIGNING_PRIVATE_KEY_PASSWORD;

function readPackageJson() {
  const content = readFileSync(PACKAGE_JSON_PATH, 'utf-8');
  return JSON.parse(content);
}

function parseVersion(version) {
  const match = version.replace(/^v/, '').match(/^(\d+)\.(\d+)\.(\d+)/);
  if (!match) return null;
  return {
    major: Number.parseInt(match[1], 10),
    minor: Number.parseInt(match[2], 10),
    patch: Number.parseInt(match[3], 10),
  };
}

function formatVersion({ major, minor, patch }) {
  return `${major}.${minor}.${patch}`;
}

function isValidVersion(version) {
  return /^\d+\.\d+\.\d+$/.test(version.replace(/^v/, ''));
}

function compareVersions(a, b) {
  const va = parseVersion(a);
  const vb = parseVersion(b);
  if (!va || !vb) return 0;
  if (va.major !== vb.major) return va.major - vb.major;
  if (va.minor !== vb.minor) return va.minor - vb.minor;
  return va.patch - vb.patch;
}

function bumpMajor(version) {
  const v = parseVersion(version);
  return formatVersion({ major: v.major + 1, minor: 0, patch: 0 });
}

function bumpMinor(version) {
  const v = parseVersion(version);
  return formatVersion({ major: v.major, minor: v.minor + 1, patch: 0 });
}

function bumpPatch(version) {
  const v = parseVersion(version);
  return formatVersion({ major: v.major, minor: v.minor, patch: v.patch + 1 });
}

function generateReleaseNotes() {
  // 获取最新 tag
  const lastTagResult = spawnSync('git', ['describe', '--tags', '--abbrev=0'], {
    cwd: ROOT_DIR,
    shell: true,
    stdio: 'pipe',
  });
  const lastTag = lastTagResult.stdout?.toString().trim() || '';

  // 获取从上次 tag 到 HEAD 的提交记录
  const logArgs = lastTag
    ? ['log', `${lastTag}..HEAD`, '--oneline', '--no-merges']
    : ['log', '--oneline', '--no-merges', '-20'];

  const logResult = spawnSync('git', logArgs, {
    cwd: ROOT_DIR,
    shell: true,
    stdio: 'pipe',
  });

  const logOutput = logResult.stdout?.toString().trim() || '';
  if (!logOutput) {
    return '';
  }

  // 按类型分组
  const groups = {
    feat: [],
    fix: [],
    refactor: [],
    perf: [],
    pref: [],
    style: [],
    chore: [],
    other: [],
  };

  for (const line of logOutput.split('\n')) {
    const msg = line.replace(/^[a-f0-9]+\s+/, '').trim();
    const match = msg.match(/^(feat|fix|refactor|perf|pref|style|chore)/);
    if (match) {
      groups[match[1]].push(msg);
    } else {
      groups.other.push(msg);
    }
  }

  // 格式化为 Markdown
  const sections = [];
  const typeLabels = {
    feat: '新功能',
    fix: '问题修复',
    refactor: '重构',
    perf: '性能优化',
    pref: '性能优化',
    style: '代码格式',
    chore: '杂项',
    other: '其他',
  };

  for (const [type, items] of Object.entries(groups)) {
    if (items.length > 0) {
      sections.push(`## ${typeLabels[type]}`);
      for (const item of items) {
        sections.push(`- ${item}`);
      }
    }
  }

  return sections.join('\n');
}

function findSignatureFile(version) {
  const sigPath = join(
    ROOT_DIR,
    'src-tauri',
    'target',
    'release',
    'bundle',
    'nsis',
    `QuickLauncher_${version}_x64-setup.exe.sig`,
  );
  try {
    return readFileSync(sigPath, 'utf-8').trim();
  } catch {}
  return '';
}

function updateCargoVersion(newVersion) {
  const content = readFileSync(CARGO_TOML_PATH, 'utf-8');
  const updated = content.replace(/^(version\s*=\s*)"[^"]*"/m, `$1"${newVersion}"`);
  writeFileSync(CARGO_TOML_PATH, updated);
}

function updateTauriConfVersion(newVersion) {
  const content = readFileSync(TAURI_CONF_PATH, 'utf-8');
  const updated = content.replace(/("version"\s*:\s*)"[^"]*"/, `$1"${newVersion}"`);
  writeFileSync(TAURI_CONF_PATH, updated);
}

async function main() {
  const pkg = readPackageJson();
  const currentVersion = pkg.version;

  console.log(`\n当前版本: v${currentVersion}\n`);

  const choice = await select({
    message: '请选择版本更新类型:',
    choices: [
      {
        name: `重大更新 (Major)    v${currentVersion} -> v${bumpMajor(currentVersion)}`,
        value: 'major',
      },
      {
        name: `功能更新 (Minor)    v${currentVersion} -> v${bumpMinor(currentVersion)}`,
        value: 'minor',
      },
      {
        name: `次要更新 (Patch)    v${currentVersion} -> v${bumpPatch(currentVersion)}`,
        value: 'patch',
      },
      {
        name: '自定义版本号',
        value: 'custom',
      },
      {
        name: '跳过版本更新 (使用当前版本)',
        value: 'skip',
      },
    ],
  });

  let newVersion = '';

  switch (choice) {
    case 'major':
      newVersion = bumpMajor(currentVersion);
      break;
    case 'minor':
      newVersion = bumpMinor(currentVersion);
      break;
    case 'patch':
      newVersion = bumpPatch(currentVersion);
      break;
    case 'custom': {
      const customInput = await input({
        message: '请输入自定义版本号 (格式: x.y.z):',
        validate: value => {
          const clean = value.replace(/^v/, '');
          if (!isValidVersion(clean)) {
            return '无效的版本号格式，请使用 x.y.z 格式';
          }
          if (compareVersions(clean, currentVersion) <= 0) {
            return `自定义版本号 (v${clean}) 必须大于当前版本号 (v${currentVersion})`;
          }
          return true;
        },
      });
      newVersion = customInput.replace(/^v/, '');
      break;
    }
    case 'skip':
      newVersion = currentVersion;
      break;
  }

  if (choice === 'skip') {
    console.log(`\n跳过版本更新，使用当前版本: v${currentVersion}\n`);
  } else {
    console.log(`\n版本更新: v${currentVersion} -> v${newVersion}\n`);

    // 更新 package.json 版本号
    pkg.version = newVersion;
    writeFileSync(PACKAGE_JSON_PATH, `${JSON.stringify(pkg, null, 2)}\n`);
    console.log('已更新 package.json 版本号');

    // 更新 Cargo.toml 版本号
    updateCargoVersion(newVersion);
    console.log('已更新 src-tauri/Cargo.toml 版本号');

    // 更新 tauri.conf.json 版本号
    updateTauriConfVersion(newVersion);
    console.log('已更新 src-tauri/tauri.conf.json 版本号');
  }

  // 执行 tauri build
  console.log('\n开始执行 pnpm tauri build ...\n');
  const buildResult = spawnSync('pnpm', ['tauri', 'build', '--ci'], {
    cwd: ROOT_DIR,
    stdio: 'inherit',
    shell: true,
    env: {
      ...process.env,
      TAURI_SIGNING_PRIVATE_KEY,
      TAURI_SIGNING_PRIVATE_KEY_PASSWORD,
    },
  });
  if (buildResult.status !== 0) {
    console.error('\n构建失败');
    process.exit(1);
  }

  // 读取构建产物的签名
  const signature = findSignatureFile(newVersion);
  if (!signature) {
    console.warn('\n警告: 未找到签名文件 (.sig)，latest.json 中的 signature 为空');
  } else {
    console.log(`\n签名内容: ${signature}`);
  }

  // 生成发布说明
  const notes = generateReleaseNotes();
  if (notes) {
    console.log('\n发布说明:\n');
    console.log(notes);
  }

  // 生成 latest.json
  const latestJson = {
    version: `v${newVersion}`,
    notes: notes || `Release v${newVersion}`,
    pub_date: new Date().toISOString(),
    platforms: {
      'windows-x86_64': {
        signature,
        url: `https://github.com/Peng1203/QuickLauncher/releases/download/v${newVersion}/QuickLauncher_${newVersion}_x64-setup.exe`,
      },
    },
  };

  writeFileSync(LATEST_JSON_PATH, `${JSON.stringify(latestJson, null, 2)}\n`);
  console.log('\n已生成 latest.json');

  // 询问是否提交版本文件和创建 tag
  if (choice !== 'skip') {
    const shouldCommit = await confirm({
      message: '是否提交版本文件并创建 git tag?',
      default: true,
    });

    if (shouldCommit) {
      // git commit
      spawnSync('git', ['add', 'package.json', 'src-tauri/Cargo.toml', 'src-tauri/Cargo.lock', 'latest.json'], {
        cwd: ROOT_DIR,
        shell: true,
        stdio: 'inherit',
      });
      spawnSync('git', ['commit', '-m', `chore: release version ${newVersion}`], {
        cwd: ROOT_DIR,
        shell: true,
        stdio: 'inherit',
      });
      console.log('\n已提交版本文件');

      // git tag
      spawnSync('git', ['tag', '-a', `v${newVersion}`, '-m', `chore: release version ${newVersion}`], {
        cwd: ROOT_DIR,
        shell: true,
        stdio: 'inherit',
      });
      console.log(`\n已创建 git tag: v${newVersion}`);
    }
  }

  console.log('\n构建完成!');

  // 打开 GitHub 新建发布页面
  if (choice !== 'skip') {
    spawnSync('start', ['https://github.com/Peng1203/QuickLauncher/releases/new'], {
      shell: true,
      stdio: 'ignore',
    });
  }
}

main();

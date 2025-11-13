import antfu from '@antfu/eslint-config';

export default antfu({
  vue: true,
  typescript: true,
  rules: {
    'no-console': 'off',
    'vue/block-order': ['error', { order: ['template', 'script', 'style'] }],
    'unused-imports/no-unused-vars': 'error',
    'no-unused-vars': 'error',
    'vue/no-unused-refs': 'off',
    'antfu/if-newline': 'off',
    'style/semi': ['error', 'always'],
    'style/brace-style': 'off',
    'style/comma-dangle': 'off',
    'style/operator-linebreak': 'off',
    'style/arrow-parens': 'off',
    'vue/html-self-closing': [
      'error',
      {
        html: {
          void: 'always',
          normal: 'never',
          component: 'always',
        },
        svg: 'always',
        math: 'always',
      },
    ],
    'vue/singleline-html-element-content-newline': 'off',
  },
});

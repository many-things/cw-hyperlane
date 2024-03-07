module.exports = {  
	root: true,
	// eslint에 어떠한 parser를 사용할지 알려주는 옵션
	// eslint가 typescript 문법을 이해할 수 있게 해준다.
	parser: '@typescript-eslint/parser',
	// typescript-eslint에서 제공하는 규칙들을 사용할 수 있게 해준다.
	plugins: ['@typescript-eslint'],  
	// 어떠한 규칙들과 설정으로 eslint를 사용할지 명시한다.
	// 아래와 같이 작성하면 default 값으로 적용이 된다.
	extends: [
		'eslint:recommended',
		'plugin:@typescript-eslint/recommended',
        'plugin:prettier/recommended'  
	],

	rules: {
		// 세미콜론이 없으면 에러로 취급한다.
		semi: 'error',
		// 기존 프로젝트에서는 'warn'으로 취급되지만, 'error'로 설정하면 에러로 취급한다.
		'@typescript-eslint/no-unused-vars': 'error'
	}
};

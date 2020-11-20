module.exports = {
	publicPath: '',
	devServer: {
		port: 8080
	},
	pages: {
		index: {
			entry: 'src/index/main.js',
			template: 'public/index.html',
			filename: 'index.html',
			title: 'Soundloop',
		},
	}
}
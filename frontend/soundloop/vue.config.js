module.exports = {
  "publicPath": "",
  "devServer": {
    "port": 8080
  },
	"assetsDir": "assets",
  "pages": {
    "index": {
      "entry": "src/index/main.js",
      "template": "public/index.html",
      "filename": "main/index.html",
      "title": "Soundloop"
    },
    "login": {
      "entry": "src/login/main.js",
      "template": "public/index.html",
      "filename": "login/login.html",
      "title": "Soundloop - Login"
    },
    "register": {
      "entry": "src/register/main.js",
      "template": "public/index.html",
      "filename": "login/register.html",
      "title": "Soundloop - Register"
    }
  },
}

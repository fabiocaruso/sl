<template>
	<div id="content">
		<header>
			<h1>Soundloop</h1>
			<nav class="nav">
			</nav>
			<div class="account">
				<!--<img src="images/avatar.png"/>-->
				<i class="fa fa-angle-down"></i>
			</div>
		</header>
		<section class="playlist">
		</section>
		<section class="main">
			<MusicList :apiUrl="apiUrl" :playlist="currentPlaylist" class="list"></MusicList>
		</section>
		<section class="downloadlist">
			<list :apiUrl="apiUrl" class="list"></list>
		</section>
		<footer>
		</footer>
	</div>
</template>

<script>
import list from './components/List.vue'
import MusicList from './components/MusicList.vue'

let url = process.env.VUE_APP_API_URL;
if (url == undefined) {
	url = ""
}

export default {
	name: 'App',
	components: {
		list,
		MusicList,
	},
	data() {
		return {
			apiUrl: url,
			currentPlaylist: "*",
		}
	},
}
</script>

<style>
*{
	padding:0;
	margin:0;
}

html, body, #app {
	width: 100%;
	height: 100%;
}

#app {
	font-family: Avenir, Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-align: center;
	color: #2c3e50;
}

#content {
	height: 100vh;
	display: grid;
	grid-template-columns: var(--layout-chat-width) repeat(8, 1fr) var(--layout-playlist-width);
	grid-template-rows: var(--layout-header-height) repeat(8, 1fr) var(--layout-footer-height);
}

:root {
	--layout-header-height: 4vmax;
	--layout-footer-height: 4vmax;
	--layout-chat-width: 15vw;
	--layout-playlist-width: 15vw;
}

header {
	grid-column: 1 / 11;
	grid-row: 1;
	background: #568bdf;
	display: flex;
	justify-content: space-between;
}

header > h1 {
	font-size: calc(var(--layout-header-height) * 0.75);
	margin-left: 1vw;
	color: white;
}

header > div.account {
	display: flex;
	flex-flow: row;
	align-items: center;
}

header > div.account > img {
	width: calc(var(--layout-header-height) * 0.6);
	height: calc(var(--layout-header-height) * 0.6);
	border-radius: calc(var(--layout-header-height) * 0.3);
	float: left;
}

header > div.account > i {
	font-size: calc(var(--layout-header-height) * 0.2);
	color: white;
	margin: 10px;
}

section.playlist {
	grid-column: 1;
	grid-row: 2 / 10;
	background: #ededed;
	display: flex;
	flex-direction:column;
}

section.main {
	grid-column: 2 / 10;
	grid-row: 2 / 10;
}

section.downloadlist {
	grid-column: 10 / 11;
	grid-row: 2 / 10;
	background: #595959;
	display: flex;
	flex-direction:column;
}

footer {
	grid-column: 1 / 11;
	grid-row: 10 / 11;
	background: #a8a8a8;
}
</style>

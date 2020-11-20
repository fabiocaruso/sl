<template>
  <div class="list">
		<ul>
			<li :key="video.link" v-for="video in videos">
				<img :src="video.thumbnail">
				<p>{{ video.name }}</p>
			</li>
		</ul>
		<div ref="addContainer" class="addContainer">
			<input @keyup.enter="submitVideo" placeholder="Youtube Link">
			<button @click="download">Download</button>
		</div>
  </div>
</template>

<script>
export default {
  name: 'list',
  props: {
    apiUrl: String
	},
	data() {
		return {
			videos: [],
		}
	},
	methods: {
		async getVideosList() {
			let result = await fetch(this.$props.apiUrl + '/queue')
				.catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e);
				});
			return result === undefined ? false : await result.json();
		},
		async getVideoMeta(url) {
			let videoId = (new URL(url)).searchParams.get('v');
			let proxyUrl = 'https://cors-anywhere.herokuapp.com/';
			let title = await fetch(proxyUrl + 'https://www.youtube.com/oembed?url=' + encodeURIComponent(url) + '&format=json')
				.catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e); 
				});
			title = title === undefined ? 'TITLE' : (await title.json()).title;
			return {
				status: 0, // 0 -> 'Loaded'; 1 -> 'Loading'
				link: url,
				name: title, 
				thumbnail: 'https://img.youtube.com/vi/' + videoId + '/mqdefault.jpg'
			}
		},
		async addVideo(video) {
			let response = await fetch(this.$props.apiUrl + '/addVideo', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(video),
			}).catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e); 
			});
			return response === undefined ? false : await response;
		},
		refreshVideosList() {
			this.getVideosList().then((list) => {
				if (list === false) return false;
				let isLoading = this.$data.videos.filter(video => video.status === 1).length >= 1;
				isLoading ? this.$data.videos = [{status: 1, name: 'Loading..'}] : this.$data.videos = [];
				list.queue.forEach((item) => {
					console.log(item)
					isLoading ? this.$data.videos.unshift(item) : this.$data.videos.push(item);
				});
			});
		},
		download() {
			let input = this.$refs.addContainer.querySelector("input");
			let link = input.value;
			input.value = "";
			let index = this.videos.push({status: 1,name: 'Loading..'}) - 1;
			this.getVideoMeta(link).then((video) => {
				console.log(video);
				this.videos[index] = video;
				this.addVideo(video);
			});
		},
		submitVideo() {
			this.download();
		}
	},
	mounted() {
		this.refreshVideosList();
		setInterval(function() {
			this.refreshVideosList();
		}.bind(this), 3000);
	},
}
</script>

<style scoped>
.list {
	width: 40vw;
	height: 40vh;
	background: #ededed;
}

.list > ul {
	list-style-type: none;
	padding: 0;
	margin: 0;
	overflow-y: scroll;
	height: inherit;
}

.list > ul > li {
	height: 100px;
	text-align: left;
	display: flex;
	align-items: center;
}

.list > ul > li > p {
	margin-left: 10px;
}

.list > ul > li > img {
	height: inherit;
}

.list > .addContainer {
	display: flex;
	padding: 15px;
	background: #d8d8d8;
}

.list > .addContainer > input {
	width: 100%;
	padding: 8px;
}
</style>

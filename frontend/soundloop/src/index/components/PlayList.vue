<template>
  <div class="list">
		<ul>
			<li :key="video.link" v-for="video in videos">
				<img :src="video.meta.thumbnail">
				<p>{{ video.meta.title }}</p>
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
		addVideo(url) {
			let response = fetch(this.$props.apiUrl + '/api/v1/addVideo/' + encodeURIComponent(url), {
				method: 'POST'
			})
				.then(data => data.json())
				.then(resp => {
					console.log(resp);
				})
				.catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e); 
				});
			return response === undefined ? false : response;
		},
		refreshVideosList() {
			fetch(this.$props.apiUrl + '/api/v1/queue')
				.then(resp => resp.json())
				.then(list => {
					if (list === false) return false;
					this.$data.videos = [];
					list.forEach((item) => {
						this.$data.videos.push(item.Download);
					});
				})
				.catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e);
				});
		},
		download() {
			let input = this.$refs.addContainer.querySelector("input");
			let link = input.value;
			input.value = "";
			this.videos.push({meta:{title: 'Loading..'}});
			this.addVideo(link);
			this.refreshVideosList();
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

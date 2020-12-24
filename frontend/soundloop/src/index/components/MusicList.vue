<template>
  <div class="list">
		<ul>
			<li :key="track.id" v-for="track in music">
				<!--<img :src="track.meta.thumbnail">-->
				<p>{{ track.file }}</p>
				<p style="color: red;">Status: {{ track.status.type }}</p>
			</li>
		</ul>
  </div>
</template>

<script>
export default {
  name: 'MusicList',
  props: {
		apiUrl: String,
		playlist: String,
	},
	data() {
		return {
			music: [],
		}
	},
	methods: {
		getMusic(playlist) {
			fetch(this.$props.apiUrl + '/api/v1/user/music/' + playlist)
				.then(resp => resp.json())
				.then(list => {
					if (list === false) return false;
					this.$data.music = list.music;
					console.log(list);
				})
				.catch((e) => {
					console.log("Connection lost.. trying to reconnect!");
					console.log("Error Message: " + e);
				});
		},
	},
	watch: {
		playlist: {
			immediate: true, 
			handler (playlist) {
				this.getMusic(playlist);
			}
		}
	},
	mounted() {
	},
}
</script>

<style scoped>
.list {
	background: white;
	height: 100%;
}

.list > ul {
	list-style-type: none;
	padding: 0;
	margin: 0;
	overflow-y: auto;
	height: inherit;
}

.list > ul > li:nth-child(2n) {
	background: #f9f9f9;
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
</style>

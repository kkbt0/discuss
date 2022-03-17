<template>
  <div class="hello">
    <el-link  @click="handleChange(1)"><h1>{{ msg }}</h1></el-link>
    <el-divider />
    <h2>主题</h2>
    <el-input-number v-model="num" :min="1" :max:="1000" @change="handleChange(this.num)" />
    <p><b>{{ dis_main.id }} {{ dis_main.author }}</b><br>
        {{ dis_main.content }}<br>
    </p>
    <div v-if="dis_main.son_nodes!=''"><div style="color:gray">回复信息id: {{ dis_main.son_nodes }}</div></div>
    <h2>回复</h2>
    <div v-if="child_dis==0"><el-empty description="无回复" /></div>
    <div v-for="item in child_dis" :key="item.id">
        {{ item.id }}<b>{{ item.author }}</b>&nbsp;&nbsp;to&nbsp;&nbsp;{{ item.reply_to }}<br>
        <p v-html="item.content"></p>
        <div v-if="item.son_nodes!=''">
          <el-link  @click="handleChange(item.id)">查看回复</el-link>  
        </div>
        <div><br></div>
    </div >
    <br>
    <el-input
    v-model="textarea"
    placeholder="输入回复内容"
    autosize
    type="textarea"
    />
    <div><br></div>
    <el-button @click="post_discussion">新增</el-button>
    <el-button @click="get_discussion(this.num)">刷新</el-button>
  </div>
</template>
<script>
import axios from "axios"
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  },
    mounted() {
this.get_discussion(1)
},

  data() {
      return {
          dis_main: {
            id: 1,
            author: "恐咖兵糖",
            content: "Welcome to the world!",
            created_at: "0",
            up: 0,down: 0,read_number: 0,sharded_number: 0,reply_to: 0,
            father_nodes: 0,
            son_nodes: "Null"
        },
        child_dis: 0,
        textarea: "Hello world!",
        num: 1,
      }
  },
  methods: {
      jump_to_index() { this.num = 1},
      get_discussion(x) {
          axios.get("https://api.ftls.xyz/discussion/"+x).then(res => {
              this.dis_main = res.data.main_dis
              this.child_dis = res.data.child_dis
          })
      },
      post_discussion() {
          var data = {
              father_node: this.num,
              content: this.textarea,
              author: "恐咖兵糖(回复)",
              reply_to: 0
        }
        console.log(data)
          axios.post("https://api.ftls.xyz/dispost",data,{timeout:1000,headers: {'Content-Type': 'application/json'}}).then(res => {
              console.log(res)
          })
        this.get_discussion(this.num)
        },
     handleChange(x) {
         console.log(x)
         this.get_discussion(x)
     }
    }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>

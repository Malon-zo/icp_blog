<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const blogs = ref([]);
const blogTags = ref([]);

async function handleSubmit(e) {
  const target = e.target;
  const title = target.querySelector("#title").value;
  const content = target.querySelector("#content").value;
  const tags = blogTags.value;

  console.log(tags)

  await my_project_backend.add_blog(title, content, tags);
  await fetchBlogs();
};

async function fetchBlogs() {
  const rawBlogs = await my_project_backend.get_blogs();
  blogs.value = rawBlogs.map((blog) => {
    return {
      ...blog,
      date: blog.date.toString()
    }
  })
}
fetchBlogs();

function saveTag() {
  const tagsInput = document.querySelector("#tags");

  if(tagsInput.value == "")
    return ;

  blogTags.value.push(tagsInput.value);
  tagsInput.value = "";
  console.log(blogTags);
}

</script>

<template>
  <main class="container mx-auto flex flex-row">
    <section class="m-12">
      <img src="/doomer.jpg" alt="DFINITY logo" class="mx-auto mt-4 rounded-3xl border-2 border-solid border-amber-500"/>
    <br />
    <br />

    <div class="grid gap-6">
        <div class="grid gap-1">
          <p class="text-zinc-100">Title: </p>
          <input class="w-full rounded-md bg-zinc-700 text-zinc-100 outline-none p-1" id="title" type="text" />
        </div>
        <div class="grid gap-1">
          <p class="text-zinc-100">Content: </p>
          <textarea class="w-full rounded-md bg-zinc-700 text-zinc-100 outline-none p-1" id="content" type="text" rows="4" />
        </div>
        <div class="grid gap-1">
          <p class="text-zinc-100">Tags: </p>
          <input 
            v-on:keyup.enter="saveTag"
            class="w-full rounded-md bg-zinc-700 text-zinc-100 outline-none p-1" id="tags" type="text" />
        </div>
        <div class="mx-auto">
          <button class="bg-amber-500 text-zinc-900 place-self-center rounded-md p-1" @onclick="handleSubmit" type="submit">DON'T CLICK ME!</button>
        </div>
        
        <div class="text-zinc-100">
        </div>
    </div>
    </section>

    <section class="mx-auto w-max m-12">

        <div v-for="blog in blogs" class="mx-auto text-zinc-100">
          <div class="mx-auto flex flex-row place-content-between">
            <h2 class="text-3xl mb-4">{{ blog.title }}</h2>
            <div>{{ new Date(Number(blog.date / 1_000_000)).toLocaleString() }}</div>
          </div>
          <h3 class="text-xl mb-2">{{ blog.content }}</h3>
          
          <div v-for="tag in blog.tags">
            <p>{{tag}}</p>
          </div>
        </div>

    </section>
    
  </main>
</template>

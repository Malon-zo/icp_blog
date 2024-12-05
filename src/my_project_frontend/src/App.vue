<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const blogs = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector("#title").value;
  const content = target.querySelector("#title").value;
  const tags = target.querySelector("#tags").value;

  const tag_array = tags.split(',');

  await my_project_backend.add_blog(title, content, tag_array);
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

</script>

<template>
  <main class="container mx-auto">
    <img src="/doomer.jpg" alt="DFINITY logo" class="mx-auto mt-4"/>
    <br />
    <br />

    <form action="#" @submit="handleSubmit">
      <div>
        <div><p class="text-zinc-100">Title: </p><input id="title" type="text" /></div>
        <div><p class="text-zinc-100">Content: </p><input class="w-full" id="content" type="text" /></div>
        <div><p class="text-zinc-100">Tags: </p><input id="tags" type="text" /></div>
        <button type="submit">DON'T CLICK ME!</button>
        {{ blogs }}
      </div>
    </form>
  </main>
</template>

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
  <main class="container mx-auto flex flex-row">
    <section class="m-12">
      <img src="/doomer.jpg" alt="DFINITY logo" class="mx-auto mt-4 rounded-3xl border-2 border-solid border-amber-500"/>
    <br />
    <br />

    <form class="grid gap-6" action="#" @submit="handleSubmit">
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
          <input class="w-full rounded-md bg-zinc-700 text-zinc-100 outline-none p-1" id="tags" type="text" />
        </div>
        <div class="mx-auto">
          <button class="bg-amber-500 text-zinc-900 place-self-center rounded-md p-1" type="submit">DON'T CLICK ME!</button>
        </div>
        
        <div class="text-zinc-100">
        </div>
    </form>
    </section>

    <section class="m-12">

        <div class="text-zinc-100">
          {{ blogs.content }}
        </div>

    </section>
    
  </main>
</template>

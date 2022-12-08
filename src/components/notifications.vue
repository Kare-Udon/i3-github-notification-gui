<template>
  <div>
    <div v-if="~is_error">
      <div
        v-for="notification in notification_data"
        :key="notification.id"
        class="bg-white h-12 rounded-full my-4"
      >
        <div
          id="notification-list"
          class="flex flex-row items-center flex-nowrap justify-between px-4 w-full h-full"
        >
          <img
            id="avatar"
            :src="notification.repository.owner.avatar_url"
            class="w-6 h-6"
          />
          <div class="flex flex-col grow mx-4">
            <p id="repository-name" class="text-xs text-gray-500 -mb-1">
              {{ notification.repository.full_name }}
            </p>
            <p id="notification-content" class="text-sm">
              {{ notification.subject.title }}
            </p>
          </div>
          <img
            id="type"
            :src="'/src/assets/' + notification.subject.type"
            class="w-5 h-5"
          />
        </div>
      </div>
    </div>
    <div id="error-message" v-else class="mx-2 py-2">
      <h1 class="text-red-600 text-xl">Error</h1>
      <p class="m-2">
        Sorry, we were unable to reach the GitHub API at this time. Please try
        again later.
      </p>
      <p class="m-2">
        Make sure you give the GitHub Access Token permission to access your
        notification.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "@vue/runtime-core";
import { NotificationModel } from "../model/notification_model";

const notification_data: Ref<[NotificationModel] | null | undefined> = ref();
const is_error = ref(false);

onMounted(() => {
  invoke("get_github_notification")
    .then((data) => {
      notification_data.value = data as [NotificationModel] | null;
      // Processing data
      // Shorten the repository name
      notification_data.value?.forEach((notification) => {
        if (notification.repository.full_name.length > 26) {
          notification.repository.full_name =
            notification.repository.full_name.slice(0, 23);
          notification.repository.full_name += "...";
        }
      });
      // Shorten the content
      notification_data.value?.forEach((notification) => {
        if (notification.subject.title.length > 30) {
          notification.subject.title = notification.subject.title.slice(0, 27);
          notification.subject.title += "...";
        }
      });
      // Replace the type icon
      notification_data.value?.forEach((notification) => {
        let type = notification.subject.type;
        switch (type) {
          case "PullRequest":
            notification.subject.type = "git-pull-request-24.svg";
            break;
          case "Issue":
            notification.subject.type = "issue-opened-24.svg";
            break;
          case "Commit":
            notification.subject.type = "git-commit-24.svg";
            break;
          default:
            notification.subject.type = "info-24.svg";
            break;
        }
      });
    })
    .catch((err) => {
      console.log(err);
    });

  if (notification_data == null) {
    is_error.value = true;
  } else {
    is_error.value = false;
  }
});
</script>

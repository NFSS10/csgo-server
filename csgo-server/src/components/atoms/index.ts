import Loader from "./loader/loader.vue";

const install = (Vue: any) => {
    Vue.component("loader", Loader);
};

export { Loader };

export default install;

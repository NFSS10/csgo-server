import Home from "./home/home.vue";

const install = (Vue: any) => {
    Vue.component("home", Home);
};

export { Home };

export default install;
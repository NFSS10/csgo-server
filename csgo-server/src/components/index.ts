import Atoms from "./atoms";

export const install = (Vue: any) => {
    Vue.use(Atoms);
};

export * from "./atoms";

export default install;
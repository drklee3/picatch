module.exports = {
    automock: false,
    setupFiles: [
        "./jestSetup.js"
    ],
    preset: "ts-jest",
    testEnvironment: "jsdom",
};

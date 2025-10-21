Object.defineProperty(window, "__DESKULPT_INTERNALS__", {
  value: {
    apisWrapper: __TEMPLATE_apis_wrapper__,
    initialSettings: __TEMPLATE_initial_settings__,
  },
  writable: false,
  configurable: false,
  enumerable: false,
});

const props = window.__DESKULPT_INTERNALS__;
Object.freeze(props);
Object.freeze(props.initialSettings);
Object.values(props.initialSettings.widgets).forEach((value) => {
  Object.freeze(value);
});

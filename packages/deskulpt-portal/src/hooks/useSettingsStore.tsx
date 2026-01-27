import { DeskulptSettings } from "@deskulpt/bindings";
import {
  PropsWithChildren,
  ReactNode,
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";
import { StoreApi, createStore, useStore } from "zustand";
import { logger } from "@deskulpt/utils";

type SettingsStore = StoreApi<DeskulptSettings.Settings>;
type SettingsStoreSelector<T> = (s: DeskulptSettings.Settings) => T;

const SettingsStoreContext = createContext<SettingsStore | null>(null);

export function SettingsStoreProvider({
  fallback,
  children,
}: PropsWithChildren<{ fallback?: ReactNode }>) {
  const [store, setStore] = useState<SettingsStore | null>(null);

  useEffect(() => {
    let cancelled = false;

    const create = async () => {
      const settings = await DeskulptSettings.Commands.read();
      const newStore = createStore(() => settings);
      if (!cancelled) {
        setStore(newStore);
      }
    };

    create().catch(logger.error);

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (store === null) {
      return;
    }

    const unlisten = DeskulptSettings.Events.update.listen((event) => {
      store.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, [store]);

  if (store === null) {
    return <>{fallback}</>;
  }

  return (
    <SettingsStoreContext.Provider value={store}>
      {children}
    </SettingsStoreContext.Provider>
  );
}

export function useSettingsStore(): DeskulptSettings.Settings;
export function useSettingsStore<T>(selector: SettingsStoreSelector<T>): T;
export function useSettingsStore<T>(selector?: SettingsStoreSelector<T>) {
  const store = useContext(SettingsStoreContext);
  if (store === null) {
    throw new Error(
      "useSettingsStore must be used within SettingsStoreProvider",
    );
  }
  return selector === undefined ? useStore(store) : useStore(store, selector);
}

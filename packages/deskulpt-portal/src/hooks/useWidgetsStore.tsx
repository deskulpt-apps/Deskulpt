import { DeskulptWidgets } from "@deskulpt/bindings";
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

type WidgetsStore = StoreApi<DeskulptWidgets.WidgetCatalog>;
type WidgetsStoreSelector<T> = (s: DeskulptWidgets.WidgetCatalog) => T;

const WidgetsStoreContext = createContext<WidgetsStore | null>(null);

export function WidgetsStoreProvider({
  fallback,
  children,
}: PropsWithChildren<{ fallback?: ReactNode }>) {
  const [store, setStore] = useState<WidgetsStore | null>(null);

  useEffect(() => {
    let cancelled = false;

    const create = async () => {
      const widgets = await DeskulptWidgets.Commands.read();
      const newStore = createStore(() => widgets);
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

    const unlisten = DeskulptWidgets.Events.update.listen((event) => {
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
    <WidgetsStoreContext.Provider value={store}>
      {children}
    </WidgetsStoreContext.Provider>
  );
}

export function useWidgetsStore(): DeskulptWidgets.WidgetCatalog;
export function useWidgetsStore<T>(selector: WidgetsStoreSelector<T>): T;
export function useWidgetsStore<T>(selector?: WidgetsStoreSelector<T>) {
  const store = useContext(WidgetsStoreContext);
  if (store === null) {
    throw new Error("useWidgetsStore must be used within WidgetsStoreProvider");
  }
  return selector === undefined ? useStore(store) : useStore(store, selector);
}

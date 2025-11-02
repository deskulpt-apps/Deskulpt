import { deskulptWidgets } from "@deskulpt/bindings";
import { useEffect } from "react";

class SetupTasks {
  private tasks = new Set<string>();
  private ready = new Set<string>();

  public register(task: string) {
    if (this.tasks.has(task)) {
      throw new Error(`Setup task "${task}" is already registered`);
    }
    this.tasks.add(task);
  }

  public complete(task: string) {
    this.ready.add(task);
    if (this.ready.size === this.tasks.size) {
      deskulptWidgets.commands.completeSetup().catch(console.error);
    }
  }

  public uncomplete(task: string) {
    this.ready.delete(task);
  }
}

const SETUP_TASKS = new SetupTasks();

export function createSetupTaskHook<T>({
  task,
  onMount,
  onUnmount,
}: {
  task: string;
  onMount: () => T;
  onUnmount?: (res: T) => void;
}) {
  SETUP_TASKS.register(task);

  return function useSetupTask() {
    useEffect(() => {
      const res = onMount();

      SETUP_TASKS.complete(task);

      return () => {
        SETUP_TASKS.uncomplete(task);
        if (onUnmount !== undefined) {
          onUnmount(res);
        }
      };
    }, []);
  };
}

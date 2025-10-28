import { useEffect } from "react";
import { toast } from "sonner";
import { deskulptCore } from "@deskulpt/bindings";

export function useShowToastListener() {
  useEffect(() => {
    const unlisten = deskulptCore.events.showToast.listen((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case "success":
          void toast.success(content);
          break;
        case "error":
          void toast.error(content);
          break;
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}

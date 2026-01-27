import { DeskulptWidgets } from "@deskulpt/bindings";
import { useCallback } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { useWidgetsGalleryStore } from "./useWidgetsGalleryStore";
import { logger } from "@deskulpt/utils";
import { toast } from "sonner";

type InstallationStatus = "installed" | "not-installed" | "upgrade-available";

export function useInstallWidget(
  reference: DeskulptWidgets.RegistryWidgetReference,
  version: string,
) {
  const localId = `@${reference.handle}.${reference.id}`;
  const localWidget = useWidgetsStore((state) => state[localId]);
  const isInFlight = useWidgetsGalleryStore((state) =>
    state.inFlightOps.has(localId),
  );

  let status: InstallationStatus;
  if (localWidget === undefined) {
    status = "not-installed";
  } else if (
    localWidget.type === "ok" &&
    localWidget.content.version === version
  ) {
    status = "installed";
  } else {
    status = "upgrade-available";
  }

  const install = useCallback(async () => {
    useWidgetsGalleryStore.getState().addInFlightOp(localId);
    try {
      await DeskulptWidgets.Commands.install(reference);
      toast.success(`Installed: ${localId}`);
    } catch (error) {
      logger.error(error);
      toast.error(`Installation failed: ${localId}`);
    } finally {
      useWidgetsGalleryStore.getState().removeInFlightOp(localId);
    }
  }, [reference, localId]);

  const uninstall = useCallback(async () => {
    useWidgetsGalleryStore.getState().addInFlightOp(localId);
    try {
      await DeskulptWidgets.Commands.uninstall(reference);
      toast.success(`Uninstalled: ${localId}`);
    } catch (error) {
      logger.error(error);
      toast.error(`Uninstallation failed: ${localId}`);
    } finally {
      useWidgetsGalleryStore.getState().removeInFlightOp(localId);
    }
  }, [reference, localId]);

  const upgrade = useCallback(async () => {
    useWidgetsGalleryStore.getState().addInFlightOp(localId);
    try {
      await DeskulptWidgets.Commands.upgrade(reference);
      toast.success(`Upgraded: ${localId}`);
    } catch (error) {
      logger.error(error);
      toast.error(`Upgrade failed: ${localId}`);
    } finally {
      useWidgetsGalleryStore.getState().removeInFlightOp(localId);
    }
  }, [reference, localId]);

  return {
    status,
    isInFlight,
    install,
    uninstall,
    upgrade,
  };
}

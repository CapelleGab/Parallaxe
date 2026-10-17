import { invoke } from "@tauri-apps/api/core";
import type { AppInfo } from "./bindings/AppInfo";
import type { IpcError } from "./bindings/IpcError";
import type { LibraryDto } from "./bindings/LibraryDto";
import type { RootDto } from "./bindings/RootDto";

export type { AppInfo, IpcError, LibraryDto, RootDto };

export function isIpcError(value: unknown): value is IpcError {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as IpcError).code === "string" &&
    typeof (value as IpcError).message === "string"
  );
}

// Commands reject with an IpcError; anything else is a bug and surfaces as a plain Error.
async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw isIpcError(error) ? error : new Error(String(error));
  }
}

export function getAppInfo(): Promise<AppInfo> {
  return call("app_info");
}

/** Opens the native folder picker. Resolves to `null` when the user cancels. */
export function createLibraryWithFolder(name: string): Promise<LibraryDto | null> {
  return call("library_create_with_folder", { name });
}

export function listLibraries(): Promise<LibraryDto[]> {
  return call("library_list");
}

export function listRoots(libraryId: string): Promise<RootDto[]> {
  return call("root_list", { libraryId });
}

/** Adds an external folder through the native picker. Resolves to `null` when cancelled. */
export function addRoot(libraryId: string): Promise<RootDto | null> {
  return call("library_add_root", { libraryId });
}

/** Message safe to show to the user, whatever the rejection was. */
export function errorMessage(error: unknown): string {
  return isIpcError(error) ? error.message : "Something went wrong. Please try again.";
}

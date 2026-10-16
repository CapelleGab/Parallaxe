import { invoke } from "@tauri-apps/api/core";

export type IpcErrorCode =
  | "INVALID_ARGUMENT"
  | "PERMISSION_DENIED"
  | "ROOT_OFFLINE"
  | "ASSET_UNAVAILABLE"
  | "FORMAT_UNSUPPORTED"
  | "DECODER_FAILED"
  | "RESOURCE_LIMIT"
  | "DATABASE_ERROR"
  | "SCAN_CANCELLED"
  | "CONFLICT";

export type IpcError = {
  code: IpcErrorCode;
  message: string;
  retryable: boolean;
  details?: Record<string, unknown>;
};

export type AppInfo = {
  name: string;
  version: string;
  coreVersion: string;
};

export function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("app_info");
}

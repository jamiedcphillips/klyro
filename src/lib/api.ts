import { invoke } from "@tauri-apps/api/tauri";
export const API = {
  addLibraryPath: (path: string) => invoke<void>("add_library_path", { path }),
  listLibraryPaths: () => invoke<string[]>("list_library_paths"),
  scanLibrary: () => invoke<number>("scan_library"),
  getContinue: () => invoke<any[]>("get_continue_watching"),
  getMovies: () => invoke<any[]>("get_movies"),
  getSeries: () => invoke<any[]>("get_series"),
  startStream: (mediaId: string, quality?: string) =>
    invoke<string>("start_stream", { mediaId, quality }),
  getHttpBase: () => invoke<string>("get_http_base"),
  updateProgress: (mediaId: string, pos: number, dur: number) =>
    invoke<void>("update_progress", { mediaId, positionSec: pos, durationSec: dur }),
};
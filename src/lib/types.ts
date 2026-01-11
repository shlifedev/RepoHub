export interface RepositoryInfo {
  id: number;
  name: string;
  path: string;
  branch: string;
  gameVersion: string;
  gameVersions: string[];
  server: string;
  serverOptions: string[];
  hasWarning: boolean;
}

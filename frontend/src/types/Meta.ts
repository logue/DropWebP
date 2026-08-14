export type Meta = {
  /** Version */
  version: string;
  /** Build date */
  date: string;
};

const Meta: Meta = {
  version: __APP_VERSION__,
  date: __BUILD_DATE__
} as const;
export default Meta;

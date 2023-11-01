export const addPad = (v: string): string => {
  const s = v.startsWith("0x") ? v.slice(2) : v;
  return s.padStart(64, "0");
};

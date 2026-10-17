// Placeholder tiles hinting at where assets will appear once indexing exists.
const ratios = [
  "aspect-square",
  "aspect-[4/3]",
  "aspect-[3/4]",
  "aspect-square",
  "aspect-[16/9]",
  "aspect-[3/4]",
  "aspect-[4/3]",
  "aspect-square",
  "aspect-[3/4]",
  "aspect-[16/9]",
  "aspect-square",
  "aspect-[4/3]",
];

export function ContactSheet() {
  return (
    <div
      aria-hidden="true"
      className="max-h-64 columns-6 gap-2 overflow-hidden [mask-image:linear-gradient(to_bottom,black_20%,transparent)]"
    >
      {ratios.map((ratio, index) => (
        <div
          key={index}
          className={`${ratio} mb-2 break-inside-avoid rounded-sm border border-border bg-elevated/40`}
        />
      ))}
    </div>
  );
}

import { motion, useMotionValue, useReducedMotion, useSpring, useTransform } from "motion/react";
import { useEffect } from "react";
import { cx } from "../../lib/cx";

type ParallaxStageProps = {
  label: string;
};

const spring = { stiffness: 120, damping: 20, mass: 0.6 };
const tiles = ["aspect-square", "aspect-[3/4]", "aspect-square", "aspect-[4/3]"];

/** Two panes at different depths that drift with the pointer: the brand, made literal. */
export function ParallaxStage({ label }: ParallaxStageProps) {
  const reduceMotion = useReducedMotion();
  const pointerX = useMotionValue(0);
  const pointerY = useMotionValue(0);
  const x = useSpring(pointerX, spring);
  const y = useSpring(pointerY, spring);

  const backX = useTransform(x, (value) => value * -14);
  const backY = useTransform(y, (value) => value * -14);
  const frontX = useTransform(x, (value) => value * 22);
  const frontY = useTransform(y, (value) => value * 22);
  const tilt = useTransform(x, (value) => value * 4);

  useEffect(() => {
    if (reduceMotion) return;
    function track(event: PointerEvent) {
      pointerX.set((event.clientX / window.innerWidth) * 2 - 1);
      pointerY.set((event.clientY / window.innerHeight) * 2 - 1);
    }
    window.addEventListener("pointermove", track);
    return () => window.removeEventListener("pointermove", track);
  }, [reduceMotion, pointerX, pointerY]);

  const empty = label.trim() === "";

  return (
    <div
      aria-hidden="true"
      className="relative aspect-[5/4] w-full max-w-[26rem] [perspective:1200px]"
    >
      <motion.div
        initial={reduceMotion ? false : { x: 40, y: 30, opacity: 0 }}
        animate={{ x: 0, y: 0, opacity: 1 }}
        transition={{ duration: 0.9, ease: [0.22, 1, 0.36, 1] }}
        // panes sit at distinct depths so the tilted front pane never cuts through the back one
        style={reduceMotion ? {} : { translateX: backX, translateY: backY, z: -80 }}
        className="absolute inset-0 -translate-x-[8%] -translate-y-[8%] rounded-[28px] border-2 border-accent/50"
      />
      <motion.div
        initial={reduceMotion ? false : { x: -30, y: -20, opacity: 0 }}
        animate={{ x: 0, y: 0, opacity: 1 }}
        transition={{ duration: 0.9, ease: [0.22, 1, 0.36, 1] }}
        style={reduceMotion ? {} : { translateX: frontX, translateY: frontY, rotateY: tilt, z: 40 }}
        className="absolute inset-0 flex translate-x-[8%] translate-y-[8%] flex-col justify-between rounded-[28px] border border-border bg-elevated p-7 [box-shadow:var(--pane-shadow)]"
      >
        <div className="grid grid-cols-4 gap-2 opacity-70">
          {tiles.map((ratio, index) => (
            <div key={index} className={cx(ratio, "self-start rounded-md bg-surface")} />
          ))}
        </div>
        <p
          className={cx(
            "line-clamp-3 font-display text-4xl leading-[1.05] font-semibold tracking-tight break-words [font-stretch:85%]",
            empty && "text-fg-muted",
          )}
        >
          {empty ? "Untitled library" : label}
        </p>
      </motion.div>
    </div>
  );
}

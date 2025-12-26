import { Link, createFileRoute } from "@tanstack/react-router";
import { HomeLayout } from "fumadocs-ui/layouts/home";
import { baseOptions } from "@/lib/layout.shared";
import AnimatedGridPattern from "@/components/ui/animated-grid-pattern";
import {
  ArrowUpRightIcon,
  DownloadIcon,
  GlobeIcon,
  PaintbrushIcon,
  PlugIcon,
  TimerIcon,
  UsersIcon,
  ZapIcon,
} from "lucide-react";

export const Route = createFileRoute("/")({
  component: Home,
});

function Home() {
  return (
    <HomeLayout {...baseOptions()}>
      <Hero />
      <Features />
    </HomeLayout>
  );
}

function Hero() {
  return (
    <div className="relative flex min-h-[calc(100dvh-3.5rem)] items-center justify-center px-6">
      <AnimatedGridPattern
        numSquares={30}
        maxOpacity={0.1}
        duration={2}
        className="inset-0 skew-y-8 mask-[radial-gradient(50vw_circle_at_center,white,transparent)]"
      />
      <div className="relative z-10 max-w-xl text-center">
        <h1 className="mt-6 text-3xl font-semibold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl">
          <div className="mr-16">Your Desktop,</div>
          <div className="ml-16 animate-shimmer bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) bg-clip-text pb-2 text-transparent">
            Reimagined.
          </div>
        </h1>
        <p className="mt-10 text-fd-foreground/80 md:text-lg">
          <span className="font-bold text-fd-primary">Deskulpt</span> customizes
          your desktop with flexible and beautiful widgets, empowering you to
          create a workspace that inspires productivity and creativity.
        </p>
        <div className="mt-12 flex items-center justify-center gap-4">
          <Link
            to="/releases/latest"
            className="inline-flex animate-shimmer items-center justify-center gap-2 rounded-full bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) px-4 py-2 font-medium text-fd-primary-foreground"
          >
            <DownloadIcon size={20} /> Download Now
          </Link>
          <Link
            to="/docs/$"
            params={{ _splat: "guide" }}
            className="inline-flex items-center justify-center gap-2 rounded-full border border-fd-primary px-4 py-2 font-medium hover:bg-fd-primary/10"
          >
            Get Started <ArrowUpRightIcon size={20} />
          </Link>
        </div>
        <p className="mt-4 text-sm text-fd-foreground/80">
          Available for Windows, macOS, and Linux.
        </p>
      </div>
    </div>
  );
}

const features = [
  {
    icon: TimerIcon,
    title: "Within Seconds",
    description:
      "Install each widget with just one click. Widgets come with sane defaults to get you started right away. Customize later only as you wish.",
  },
  {
    icon: PaintbrushIcon,
    title: "Customize Every Corner",
    description:
      "You own the widgets. Change settings, configurations, and even source code as you wish. Make them truly suit every need.",
  },
  {
    icon: GlobeIcon,
    title: "Bring the Web to Desktop",
    description:
      "Widgets are written in React and rendered with web technologies. Whatever works on the web, works on your desktop.",
  },
  {
    icon: PlugIcon,
    title: "Deep System Integration",
    description:
      "Extend widgets' capabilities beyond the web. Seamlessly integrate with the system resources and APIs through our plugin system.",
  },
  {
    icon: ZapIcon,
    title: "Lightweight & Performant",
    description:
      "Deskulpt is built with performance in mind. Enjoy a smooth experience without compromising system resources.",
  },
  {
    icon: UsersIcon,
    title: "Thriving Community",
    description:
      "Deskulpt is free and open-source, with a thriving community building our gallery. Join us and share your creativity with the world!",
  },
];

function Features() {
  return (
    <div className="py-20">
      <h2 className="text-center text-3xl font-medium tracking-tight sm:text-4xl">
        <div className="md:mr-20">
          Unleash Your{" "}
          <span className="animate-shimmer bg-linear-to-r from-primary-shifted via-fd-primary to-primary-shifted bg-size-(--bg-size-shimmer) bg-clip-text font-semibold text-transparent">
            Creativity,
          </span>
        </div>
        <div className="mt-1 md:ml-20">
          Unlock Every{" "}
          <span className="animate-shimmer bg-linear-to-r from-fd-primary via-primary-shifted to-fd-primary bg-size-(--bg-size-shimmer) bg-clip-text font-semibold text-transparent">
            Possibility.
          </span>
        </div>
      </h2>

      <div className="mx-auto mt-12 grid max-w-(--breakpoint-lg) gap-4 px-6 sm:grid-cols-2 lg:grid-cols-3">
        {features.map((feature) => (
          <div
            key={feature.title}
            className="group relative top-0 flex flex-col overflow-hidden rounded-xl border bg-linear-to-b from-fd-muted/60 to-fd-card px-5 py-6 transition-all duration-300 hover:-top-1 hover:border-fd-primary/30 hover:shadow-lg"
          >
            <AnimatedGridPattern
              width={24}
              height={24}
              numSquares={18}
              maxOpacity={0.3}
              animate={false}
              className="pointer-events-none absolute inset-0 -z-10 mask-[radial-gradient(circle_at_top,white,transparent_70%)] text-fd-primary/25"
            />

            <div className="absolute inset-x-0 bottom-0 h-1.5 bg-linear-to-r from-transparent via-fd-primary to-transparent opacity-0 blur-[1px] transition-opacity duration-500 group-hover:opacity-100" />

            <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-fd-muted transition-all duration-300 group-hover:scale-125 group-hover:bg-fd-primary/10 group-hover:text-fd-primary">
              <feature.icon className="size-5 transition-colors" />
            </div>
            <span className="text-lg font-semibold">{feature.title}</span>
            <p className="mt-2 text-[15px] text-fd-foreground/80">
              {feature.description}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}

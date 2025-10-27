import { defineConfig } from "@pandacss/dev";

export default defineConfig({
  // Whether to use css reset
  preflight: true,

  // Where to look for your css declarations
  include: ["./src/**/*.{js,ts,svelte}"],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {
      tokens: {
        colors: {
          // Void - Bachelardian intimate darkness palette
          void: {
            50: { value: "#f8fafc" },
            100: { value: "#f1f5f9" },
            200: { value: "#e2e8f0" },
            300: { value: "#cbd5e1" },
            400: { value: "#94a3b8" },
            500: { value: "#64748b" },
            600: { value: "#475569" },
            700: { value: "#334155" },
            800: { value: "#1e293b" },
            900: { value: "#0f172a" },
            950: { value: "#020617" },
          },
          // Breakthrough - Electric cyan for moments of clarity
          breakthrough: {
            50: { value: "#ecfeff" },
            100: { value: "#cffafe" },
            200: { value: "#a5f3fc" },
            300: { value: "#67e8f9" },
            400: { value: "#22d3ee" },
            500: { value: "#06b6d4" },
            600: { value: "#0891b2" },
            700: { value: "#0e7490" },
            800: { value: "#155e75" },
            900: { value: "#164e63" },
            950: { value: "#083344" },
          },
          // Terminal - Phosphorescent glow colors
          terminal: {
            glow: { value: "#00ffff" },
            pulse: { value: "rgba(6, 182, 212, 0.5)" },
            shadow: { value: "rgba(34, 211, 238, 0.25)" },
          },
          // Liminal - Semi-transparent transformation states
          liminal: {
            surface: { value: "rgba(30, 41, 59, 0.7)" },
            overlay: { value: "rgba(15, 23, 42, 0.5)" },
            border: { value: "rgba(71, 85, 105, 0.3)" },
            hover: { value: "rgba(6, 182, 212, 0.1)" },
          },
        },
        shadows: {
          glow: { value: "0 0 20px rgba(34, 211, 238, 0.25)" },
          pulse: { value: "0 0 30px rgba(6, 182, 212, 0.4)" },
          breakthrough: { value: "0 10px 40px rgba(34, 211, 238, 0.15)" },
          void: { value: "0 25px 50px -12px rgba(0, 0, 0, 0.8)" },
        },
        animations: {
          pulseGlow: {
            value: "pulse-glow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
          },
          flicker: { value: "flicker 3s ease-in-out infinite" },
          terminalBlink: { value: "terminal-blink 1s step-end infinite" },
        },
        blurs: {
          liminal: { value: "4px" },
        },
      },
      semanticTokens: {
        colors: {
          // Background colors
          bg: {
            primary: { value: "{colors.void.950}" },
            secondary: { value: "{colors.void.900}" },
            muted: { value: "{colors.void.800}" },
            accent: { value: "{colors.breakthrough.400}" },
          },
          // Text colors
          text: {
            primary: { value: "{colors.void.50}" },
            secondary: { value: "{colors.void.300}" },
            muted: { value: "{colors.void.400}" },
            accent: { value: "{colors.breakthrough.400}" },
            glow: { value: "{colors.terminal.glow}" },
          },
          // Border colors
          border: {
            default: { value: "{colors.void.600}" },
            hover: { value: "{colors.terminal.pulse}" },
            active: { value: "{colors.breakthrough.400}" },
            liminal: { value: "{colors.liminal.border}" },
          },
        },
      },
      keyframes: {
        pulseGlow: {
          "0%, 100%": { opacity: "1", boxShadow: "0 0 20px rgba(34, 211, 238, 0.25)" },
          "50%": { opacity: "0.8", boxShadow: "0 0 30px rgba(6, 182, 212, 0.4)" },
        },
        flicker: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0.95" },
          "75%": { opacity: "0.98" },
        },
        terminalBlink: {
          "0%, 50%": { opacity: "1" },
          "51%, 100%": { opacity: "0" },
        },
      },
    },
  },

  // The output directory for your css system
  outdir: "styled-system",
});

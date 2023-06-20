module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./docs/**/*.html"],
  theme: {
    extend: {
      keyframes: {
        "fade-in-down": {
          "0%": {
            opacity: "0",
            transform: "translateY(100%)",
          },
          "100%": {
            opacity: "1",
            transform: "translateY(0)",
          },
        },
        "fade-in-up": {
          "0%": {
            opacity: "0",
            transform: "translateY(-100%)",
          },
          "100%": {
            opacity: "1",
            transform: "translateY(0)",
          },
        },
        "fade-in-left": {
          "0%": {
            opacity: "0",
            transform: "translateX(-100%)",
          },
          "100%": {
            opacity: "1",
            transform: "translateY(0)",
          },
        },
        "fade-in-right": {
          "0%": {
            opacity: "0",
            transform: "translateX(100%)",
          },
          "100%": {
            opacity: "1",
            transform: "translateY(0)",
          },
        },
        shine: {
          "0%": {
            "background-image": "linear-gradient(90deg,#215d6e,#39b48e)",
            "background-size": "1100% 100%",
            "background-position": "0% 0%",
          },
          "50%": {
            "background-image": "linear-gradient(90deg,#215d6e,#39b48e)",
            "background-size": "1100% 100%",
            "background-position": "100% 0%",
          },
          "100%": {
            "background-image": "linear-gradient(90deg,#215d6e,#39b48e)",
            "background-size": "1100% 100%",
            "background-position": "0% 0%",
          },
        },
      },
      animation: {
        shine: "shine 10s ease-in-out infinite",
        "fade-in-down": "fade-in-down 0.5s ease-in-out",
        "fade-in-up": "fade-in-up 0.5s ease-in-out",
        "fade-in-right": "fade-in-right 0.5s ease-in-out",
        "fade-in-right-slow": "fade-in-right 0.75s ease-in-out",
        "fade-in-right-slower": "fade-in-right 1s ease-in-out",
        "fade-in-left": "fade-in-left 0.5s ease-in-out",
        "fade-in-left-slow": "fade-in-left 0.75s ease-in-out",
        "fade-in-left-slower": "fade-in-left 1s ease-in-out",
      },
    },
  },
  plugins: [],
};

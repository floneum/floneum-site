module.exports = {
  mode: "all",
  content: [
      "./src/**/*.{rs,html,css}",
      "./docs/**/*.html",
  ],
  theme: {
    extend: {
        keyframes: {
            'fade-in-down': {
                '0%': {
                    opacity: '0',
                    transform: 'translateY(100%)'
                },
                '100%': {
                    opacity: '1',
                    transform: 'translateY(0)'
                },
            },
            'fade-in-up': {
                '0%': {
                    opacity: '0',
                    transform: 'translateY(-100%)'
                },
                '100%': {
                    opacity: '1',
                    transform: 'translateY(0)'
                }
            },
            'fade-in-left': {
                '0%': {
                    opacity: '0',
                    transform: 'translateX(-100%)'
                },
                '100%': {
                    opacity: '1',
                    transform: 'translateY(0)'
                }
            },
            'fade-in-right': {
                '0%': {
                    opacity: '0',
                    transform: 'translateX(100%)'
                },
                '100%': {
                    opacity: '1',
                    transform: 'translateY(0)'
                }
            },
        },
        animation: {
            'fade-in-down': 'fade-in-down 0.5s ease-in-out',
            'fade-in-up': 'fade-in-up 0.5s ease-in-out',
            'fade-in-right': 'fade-in-right 0.5s ease-in-out',
            'fade-in-right-slow': 'fade-in-right 0.75s ease-in-out',
            'fade-in-right-slower': 'fade-in-right 1s ease-in-out',
            'fade-in-left': 'fade-in-left 0.5s ease-in-out',
            'fade-in-left-slow': 'fade-in-left 0.75s ease-in-out',
            'fade-in-left-slower': 'fade-in-left 1s ease-in-out',
        }
    },
  },
  plugins: [],
}

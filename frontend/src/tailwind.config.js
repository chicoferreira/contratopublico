/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            '--tw-prose-body': 'var(--color-foreground)',
            '--tw-prose-headings': 'var(--color-foreground)',
            '--tw-prose-lead': 'var(--color-foreground)',
            '--tw-prose-links': 'var(--color-primary)',
            '--tw-prose-links-underline': 'none',
            '--tw-prose-bold': 'var(--color-foreground)',
            '--tw-prose-counters': 'var(--color-muted-foreground)',
            '--tw-prose-bullets': 'var(--color-muted-foreground)',
            '--tw-prose-hr': 'var(--color-border)',
            '--tw-prose-quotes': 'var(--color-foreground)',
            '--tw-prose-quote-borders': 'var(--color-border)',
            '--tw-prose-captions': 'var(--color-muted-foreground)',
            '--tw-prose-code': 'var(--color-foreground)',
            '--tw-prose-pre-code': 'var(--color-foreground)',
            '--tw-prose-pre-bg': 'var(--color-muted)',
            '--tw-prose-th-borders': 'var(--color-border)',
            '--tw-prose-td-borders': 'var(--color-border)',
            color: 'var(--color-foreground)',

            p: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            a: {
              textDecoration: 'none',
            },
            'a:hover': {
              textDecoration: 'underline',
            },
            ul: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
              paddingLeft: '1.25em',
            },
            ol: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
              paddingLeft: '1.25em',
            },
            li: {
              marginTop: '0.25em',
              marginBottom: '0.25em',
            },
            pre: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            blockquote: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            table: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            img: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            video: {
              marginTop: '0.6em',
              marginBottom: '0.6em',
            },
            h1: {
              marginTop: '0',
              marginBottom: '0.6em',
              fontWeight: 'var(--font-weight-semibold)',
            },
            'h2, h3, h4, h5, h6': {
              marginTop: '0.9em',
              marginBottom: '0.5em',
              fontWeight: 'var(--font-weight-semibold)',
            },
          },
        },
      },
    },
  },
}

# Fonts

Drop self-hosted Inter woff2 subsets here:

- `inter-latin-400.woff2`
- `inter-latin-500.woff2`
- `inter-latin-600.woff2`
- `inter-latin-700.woff2`

Sources:

- Inter is OFL-1.1 licensed: https://github.com/rsms/inter
- Pre-subset latin files are available via @fontsource/inter

If these files are absent the site silently falls back to the system
sans-serif stack defined in `styles/main.css`. No third-party request
is made under any circumstance — this is required for Swiss nDSG / EU
GDPR compliance because the user did not opt in to data transmission
to a font CDN.

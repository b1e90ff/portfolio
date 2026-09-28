import { defineConfig, devices } from '@playwright/test';

const PORT = 3100;
const SWIFTSHADER = ['--use-angle=swiftshader', '--enable-unsafe-swiftshader'];

export default defineConfig({
    testDir: 'tests/e2e',
    fullyParallel: true,
    forbidOnly: Boolean(process.env.CI),
    retries: process.env.CI ? 1 : 0,
    timeout: 90_000,
    reporter: process.env.CI ? [['github'], ['list']] : 'list',
    use: {
        baseURL: `http://127.0.0.1:${PORT}`,
        trace: 'retain-on-failure',
        launchOptions: { args: SWIFTSHADER },
    },
    projects: [
        { name: 'desktop', use: { ...devices['Desktop Chrome'], viewport: { width: 1440, height: 900 } } },
        { name: 'mobile', use: { ...devices['Pixel 7'] } },
        { name: 'mobile-landscape', use: { ...devices['Pixel 7 landscape'] } },
    ],
    webServer: {
        command: 'cargo run --locked',
        url: `http://127.0.0.1:${PORT}/healthz`,
        reuseExistingServer: !process.env.CI,
        timeout: 300_000,
        env: { PORTFOLIO_BIND: `127.0.0.1:${PORT}`, PORTFOLIO_BASE_URL: `http://127.0.0.1:${PORT}`, SMTP_HOST: '' },
    },
});

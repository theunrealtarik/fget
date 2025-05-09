import { defineManifest } from '@crxjs/vite-plugin'
import packageData from '../package.json'

//@ts-ignore
const isDev = process.env.NODE_ENV == 'development'

export default defineManifest({
  name: `${packageData.displayName || packageData.name}${isDev ? ` dev` : ''}`,
  description: packageData.description,
  version: packageData.version,
  manifest_version: 3,
  options_page: 'options.html',
  background: {
    service_worker: 'src/background/index.ts',
    type: 'module',
  },
  content_scripts: [
    {
      matches: ['http://*/*', 'https://*/*'],
      js: ['src/content/index.ts'],
    },
  ],
  permissions: [
    "downloads",
    "downloads.shelf",
    "webRequest",
    "storage",
    "nativeMessaging"
  ],
  host_permissions: [
    "http://localhost/*",
    "http://127.0.0.1/*"
  ]

})

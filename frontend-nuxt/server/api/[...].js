export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const target = config.apiProxyTarget || 'http://localhost:8006'
  const path = event.path
  
  const targetUrl = `${target}${path}`
  console.log(`[Proxy] Forwarding ${path} to ${targetUrl}`)
  
  return proxyRequest(event, targetUrl)
})

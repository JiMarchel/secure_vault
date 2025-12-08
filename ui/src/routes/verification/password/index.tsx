import { createFileRoute } from '@tanstack/react-router'
import { signUpMiddleware } from '@/middleware/signup'

export const Route = createFileRoute('/verification/password/')({
  server: {
    middleware: [signUpMiddleware]
  },
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/verification/password/"!</div>
}

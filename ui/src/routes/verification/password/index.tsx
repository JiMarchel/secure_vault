import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/verification/password/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/verification/password/"!</div>
}

import {
  createFileRoute,
  redirect,
  useNavigate,
  useSearch,
} from '@tanstack/react-router'
import { AlertCircle } from 'lucide-react'
import { toast } from 'sonner'
import { Button } from '@/components/ui/button'

export const Route = createFileRoute('/internal-error/')({
  component: RouteComponent,
  validateSearch: (search: Record<string, unknown>) => ({
    error: search.error as string,
    requestId: (search.requestId as string) || undefined,
  }),
  beforeLoad: ({ search }) => {
    if (!search.error) {
      throw redirect({ to: '/' })
    }
  },
})

function RouteComponent() {
  const navigate = useNavigate()
  const { error, requestId } = useSearch({
    from: '/internal-error/',
  })

  toast.error(error, { duration: 10000 })

  return (
    <main className="flex-1 flex items-center justify-center min-h-screen bg-gray-50">
      <div className="w-full max-w-md px-4 py-8">
        <div className="bg-white rounded-lg shadow-lg p-8 text-center">
          <div className="flex justify-center mb-4">
            <div className="bg-red-100 rounded-full p-4">
              <AlertCircle className="w-8 h-8 text-red-600" />
            </div>
          </div>

          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            Oops! Server Error
          </h1>

          <p className="text-gray-600 mb-4">
            Something went wrong on our server. Please copy this Request Id and
            contact our customer support.
          </p>

          <div className="bg-gray-100 rounded p-3 mb-6">
            <p className="text-sm text-gray-700">
              <strong>Request ID:</strong> {requestId}
            </p>
          </div>

          <div className="space-y-3">
            <Button
              className="w-full bg-primary hover:bg-primary/90"
              onClick={() => navigate({ to: '/' })}
            >
              Back to Home
            </Button>
          </div>

          {process.env.NODE_ENV === 'development' && (
            <div className="mt-6 p-4 bg-yellow-50 rounded border border-yellow-200">
              <p className="text-xs text-yellow-800 font-mono wrap-break-word">
                {error}
              </p>
            </div>
          )}
        </div>
      </div>
    </main>
  )
}

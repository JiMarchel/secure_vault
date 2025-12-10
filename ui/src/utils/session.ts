import { useSession } from "@tanstack/react-start/server"
import type { SessionData } from "@/model/auth"

export function useAppSession() {
    return useSession<SessionData>({
        name: "sv_auth",
        password: process.env.PASSWORD!,
        cookie: {
            secure: process.env.NODE_ENV === "production",
            sameSite: "lax",
            httpOnly: true,
            maxAge: 60 * 60 * 24 * 7, // 7 days
        }
    })
}

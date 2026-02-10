package nuts.deez.kotlin_multiplatform

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform
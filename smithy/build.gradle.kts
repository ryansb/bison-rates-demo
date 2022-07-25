plugins {
  id("software.amazon.smithy").version("0.6.0")
  id("net.thauvin.erik.gradle.semver").version("1.0.4")
}

// The smithy plugin sets includes these dependencies automatically with version ranges
// this can cause spontaneous breakages when Smithy pushes new dependencies that match
// those ranges but are otherwise incompatible with the remaining specified dependencies.
// These are included in buildscript/dependencies to explicitly lock the dependencies
// that the plugin automatically provides.
buildscript {
    repositories {
      mavenLocal()
      gradlePluginPortal()
      mavenCentral()
    }

    val smithyVersion: String by project
    dependencies {
        classpath("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
        classpath("software.amazon.smithy:smithy-cli:$smithyVersion")
    }
}

val smithyVersion: String by project
val kotlinVersion: String by project

dependencies {
  implementation("software.amazon.smithy:smithy-cli:$smithyVersion")
  implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
  implementation("software.amazon.smithy:smithy-aws-apigateway-traits:$smithyVersion")
  implementation("software.amazon.smithy:smithy-aws-apigateway-openapi:$smithyVersion")
  implementation("software.amazon.smithy:smithy-validation-model:$smithyVersion")
  implementation("software.amazon.smithy:smithy-openapi:$smithyVersion")
  implementation("software.amazon.smithy:smithy-waiters:$smithyVersion")
  implementation("software.amazon.smithy.typescript:smithy-typescript-codegen:0.11.0")
  implementation("software.amazon.smithy.typescript:smithy-aws-typescript-codegen:0.11.0")

  // Dependencies related to Rust Code Generators
  implementation("org.jsoup:jsoup:1.14.3")
  implementation("com.moandjiezana.toml:toml4j:0.7.2")
  implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:$kotlinVersion")

  implementation(files("libs/rust-codegen-0.41.0.jar"))
  implementation(files("libs/rust-codegen-server-0.41.0.jar"))
  runtimeOnly(files("libs/rust-runtime-0.41.0.jar"))
}

repositories {
  mavenLocal()
  gradlePluginPortal()
  mavenCentral()
}

configure<software.amazon.smithy.gradle.SmithyExtension> {
  outputDirectory = file(project.getBuildDir().toPath().resolve("output").toFile())
}

tasks["jar"].enabled = false

tasks["smithyBuildJar"].doFirst {
  System.setProperty("semver.version", semver.semver)
  println("Current Version: ${semver.semver}")
}

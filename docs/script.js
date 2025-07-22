// Enhanced smooth scrolling for anchor links
document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
  anchor.addEventListener("click", function (e) {
    e.preventDefault();
    const target = document.querySelector(this.getAttribute("href"));
    if (target) {
      target.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
    }
  });
});

// Navbar background opacity on scroll
window.addEventListener("scroll", () => {
  const nav = document.querySelector(".nav");
  if (window.scrollY > 100) {
    nav.style.background = "rgba(15, 23, 42, 0.95)";
  } else {
    nav.style.background = "rgba(15, 23, 42, 0.8)";
  }
});

// Intersection Observer for fade-in animations
const observerOptions = {
  threshold: 0.1,
  rootMargin: "0px 0px -50px 0px",
};

const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.style.opacity = "1";
      entry.target.style.transform = "translateY(0)";
    }
  });
}, observerOptions);

// Observe feature cards, benchmark cards, and download cards
document.addEventListener("DOMContentLoaded", () => {
  const animatedElements = document.querySelectorAll(
    ".feature-card, .benchmark-card, .download-card"
  );

  animatedElements.forEach((el) => {
    el.style.opacity = "0";
    el.style.transform = "translateY(30px)";
    el.style.transition = "opacity 0.6s ease-out, transform 0.6s ease-out";
    observer.observe(el);
  });
});

// Add dynamic typing effect to hero title (optional enhancement)
function typeWriter(element, text, speed = 100) {
  let i = 0;
  element.innerHTML = "";

  function type() {
    if (i < text.length) {
      element.innerHTML += text.charAt(i);
      i++;
      setTimeout(type, speed);
    }
  }

  type();
}

// Enhanced demo animation for the before/after images
document.addEventListener("DOMContentLoaded", () => {
  const beforeImage = document.querySelector(".demo-image.before");
  const afterImage = document.querySelector(".demo-image.after");
  const arrow = document.querySelector(".demo-arrow");

  if (beforeImage && afterImage && arrow) {
    // Add hover effects
    const demoContainer = document.querySelector(".demo-container");

    demoContainer.addEventListener("mouseenter", () => {
      beforeImage.style.transform = "scale(1.05)";
      afterImage.style.transform = "scale(1.05)";
      arrow.style.transform = "scale(1.2)";
      arrow.style.color = "#3b82f6";
    });

    demoContainer.addEventListener("mouseleave", () => {
      beforeImage.style.transform = "scale(1)";
      afterImage.style.transform = "scale(1)";
      arrow.style.transform = "scale(1)";
      arrow.style.color = "#06b6d4";
    });
  }
});

// Add download tracking (for analytics if needed)
document.querySelectorAll(".download-btn").forEach((btn) => {
  btn.addEventListener("click", (e) => {
    const platform = e.target
      .closest(".download-card")
      .querySelector("h3").textContent;
    console.log(`Download clicked: ${platform}`);

    // Add visual feedback
    e.target.style.transform = "scale(0.95)";
    setTimeout(() => {
      e.target.style.transform = "scale(1)";
    }, 150);

    // Here you could add analytics tracking
    // gtag('event', 'download', { platform: platform });
  });
});

// Add copy-to-clipboard functionality for any code blocks (if added later)
function copyToClipboard(text) {
  navigator.clipboard.writeText(text).then(() => {
    // Show a toast notification or similar feedback
    console.log("Copied to clipboard:", text);
  });
}

// Performance monitoring (optional)
window.addEventListener("load", () => {
  const loadTime = performance.now();
  console.log(`Page loaded in ${Math.round(loadTime)} ms`);
});

// Add keyboard navigation support
document.addEventListener("keydown", (e) => {
  // Handle escape key to close any modals (if added later)
  if (e.key === "Escape") {
    // Close any open modals or overlays
  }

  // Handle arrow keys for navigation (if needed)
  if (e.key === "ArrowDown") {
    // Scroll to next section
  }
});

// Add mobile menu toggle (for responsive design enhancement)
function createMobileMenu() {
  const nav = document.querySelector(".nav-container");
  const navLinks = document.querySelector(".nav-links");

  if (
    window.innerWidth <= 768 &&
    !document.querySelector(".mobile-menu-toggle")
  ) {
    const toggleButton = document.createElement("button");
    toggleButton.className = "mobile-menu-toggle";
    toggleButton.innerHTML = "☰";
    toggleButton.style.cssText = `
            display: block;
            background: none;
            border: none;
            color: var(--text-primary);
            font-size: 1.5rem;
            cursor: pointer;
        `;

    toggleButton.addEventListener("click", () => {
      navLinks.style.display =
        navLinks.style.display === "flex" ? "none" : "flex";
    });

    nav.appendChild(toggleButton);
  }
}

// Initialize mobile menu on resize
window.addEventListener("resize", createMobileMenu);
document.addEventListener("DOMContentLoaded", createMobileMenu);

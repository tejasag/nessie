import styles from "../styles/Navbar.module.scss";

export const Navbar = () => {
    return (
        <div className={styles.navbar}>
            <header className={styles.header}>
                <nav className={styles.nav}>
                    <div className={styles.links}>
                        <a className={styles.link} href="/test">
                            <img
                                src="assets/nessie-logo.png"
                                alt="Nessie"/>
                        </a>
                    </div>
                    <div className={`${styles.mobileMenu}`}>
                        <a className={styles.link} href="/why">
                            Why Nessie?
                        </a>
                        <a className={styles.link} href="/discord">
                            Github
                        </a>
                        <a className={styles.link} href="/features">
                            Features
                        </a>
                        <a className={styles.link} href="/discord">
                            Discord
                        </a>
                    </div>
                    <div className={styles.links}>
                        <a className={styles.loginButton} href="/login">
                            Login
                        </a>
                    </div>
                </nav>
            </header>
        </div>
    )
}
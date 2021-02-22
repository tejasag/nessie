import styles from '../styles/Header.module.scss'

export const Header = () => {
    return (
        <div className={styles.Header}>
            {/*<img src="https://media.discordapp.net/attachments/770614948215914516/812010184846475305/nessielogo-removebg-preview.png?width=300&height=225" alt="Nessie" className={styles.image}/>*/}
            <img src="https://ae01.alicdn.com/kf/HTB1HbDae8TH8KJjy0Fiq6ARsXXao/deep-sea-Ocean-Underwater-Coral-Reef-photo-studio-background-Vinyl-cloth-High-quality-Computer-print-wall.jpg" alt="Nessie" className={styles.image}/>
            <div className={styles.Header__text}>
                <h1>Nessie</h1>
                <h3>The Chat App made for you.</h3>
            </div>
        </div>
    )
}
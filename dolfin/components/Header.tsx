// @ts-ignore
import Styles from '../styles/Header.module.scss';
import {Component} from "react";

export default class Head extends Component {
    render() {
        return (
            <div className={Styles.Header}>
                <img
                    src="https://ae01.alicdn.com/kf/HTB1HbDae8TH8KJjy0Fiq6ARsXXao/deep-sea-Ocean-Underwater-Coral-Reef-photo-studio-background-Vinyl-cloth-High-quality-Computer-print-wall.jpg"
                    alt="Nessie" className={Styles.image}/>
                <div className={Styles.Header__text}>
                    <h1>Nessie</h1>
                    <h3>The Chat App made for you.</h3>
                </div>
            </div>
        )
    }
}
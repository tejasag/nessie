import "../styles/global.scss";
import Head from "next/head";
import {Component} from "react";

export default class App extends Component<{ Component: any, pageProps: any }> {
    render() {
        let {Component, pageProps} = this.props;
        return (<>
            <Head>
                <meta charSet="utf-8"/>
                <link rel="icon" href="./favicon.ico"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#000000"/>
                <meta
                    name="description"
                    content="Nessie, the thing you need."
                />
                <title>Nessie</title>
            </Head>
            <Component {...pageProps} /></>);
    }
}
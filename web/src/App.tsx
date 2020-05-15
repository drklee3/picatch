import React from "react";
import "./styles/index.scss";

import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Album from "./components/Album";
import Footer from "./components/Footer";
import Header from "./components/Header";

function App() {
    return (
        <Router>
            <div className="container">
                <Header />
                <Switch>
                    <Route
                        path="/album/:albumPath+"
                        render={(props) => <Album {...props} />}
                    ></Route>
                    <Route
                        path="/"
                        render={(props) => <Album {...props} root={true} />}
                    ></Route>
                </Switch>
                <Footer />
            </div>
        </Router>
    );
}

export default App;

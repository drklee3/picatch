import React from "react";
import "./styles/index.scss";

import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import Album from "./components/Album";

function App() {
    return (
        <Router>
            <div>
                <header>
                    <h3>hey</h3>
                </header>
                <Switch>
                    <Route
                        path="/album/:albumPath+"
                        render={(props) => <Album {...props} />}
                    ></Route>
                    <Route path="/">
                        <p>home</p>
                    </Route>
                </Switch>
            </div>
        </Router>
    );
}

export default App;

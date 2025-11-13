// Example of high cyclomatic complexity
function processUserData(user, options) {
    if (!user) {
        throw new Error("User required");
    }

    if (user.age < 18 && !options.allowMinors) {
        return { error: "User too young" };
    }

    if (user.country === "US" || user.country === "CA") {
        if (user.state && user.state.length === 2) {
            console.log("North American user");
        }
    } else if (user.country === "UK" || user.country === "IE") {
        console.log("European user");
    }

    const result = {};

    if (options.includeEmail && user.email) {
        result.email = user.email.toLowerCase();
    }

    if (options.includePhone && user.phone) {
        result.phone = user.phone.replace(/\D/g, '');
    }

    if (user.premium || (user.credits && user.credits > 100)) {
        result.tier = "premium";
    } else if (user.credits && user.credits > 10) {
        result.tier = "standard";
    } else {
        result.tier = "basic";
    }

    for (let i = 0; i < user.preferences.length; i++) {
        const pref = user.preferences[i];
        if (pref.enabled && pref.value !== null) {
            result.preferences = result.preferences || [];
            result.preferences.push(pref);
        }
    }

    return result;
}

// Cyclomatic complexity: ~15-20

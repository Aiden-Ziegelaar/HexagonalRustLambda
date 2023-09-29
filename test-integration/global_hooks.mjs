import { readFileSync } from 'fs';

export const mochaHooks = {
    beforeAll (done) {
        let tf_state_raw = readFileSync('../infra/terraform.tfstate', 'utf8')
        let tf_state = JSON.parse(tf_state_raw)
        process.env.INF_API_ENDPOINT = tf_state.outputs.api_endpoint.value
        done()
    }
};